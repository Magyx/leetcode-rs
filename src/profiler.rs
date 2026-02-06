use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub struct CountingAllocator;

impl CountingAllocator {
    pub const fn new() -> Self {
        Self
    }
}

struct Metrics {
    current_bytes: AtomicUsize,
    peak_bytes: AtomicUsize,
    alloc_calls: AtomicUsize,
    dealloc_calls: AtomicUsize,
    realloc_calls: AtomicUsize,
}

static METRICS: Metrics = Metrics {
    current_bytes: AtomicUsize::new(0),
    peak_bytes: AtomicUsize::new(0),
    alloc_calls: AtomicUsize::new(0),
    dealloc_calls: AtomicUsize::new(0),
    realloc_calls: AtomicUsize::new(0),
};

#[derive(Copy, Clone)]
struct ScopeState {
    active: bool,
    base: usize,
    peak_delta: isize,
}

thread_local! {
    static TLS_SCOPE: Cell<ScopeState> = const {
        Cell::new(ScopeState { active: false, base: 0, peak_delta: 0 })
    };
}

#[inline]
fn tls_on_bytes_change(now_current_bytes: usize) {
    TLS_SCOPE.with(|tls| {
        let mut s = tls.get();
        if !s.active {
            return;
        }
        let delta = now_current_bytes as isize - s.base as isize;
        if delta > s.peak_delta {
            s.peak_delta = delta;
            tls.set(s);
        }
    });
}

#[inline]
fn update_peak(now: usize) {
    let mut peak = METRICS.peak_bytes.load(Ordering::Relaxed);
    while now > peak {
        match METRICS.peak_bytes.compare_exchange_weak(
            peak,
            now,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => break,
            Err(p) => peak = p,
        }
    }
}

#[inline]
fn apply_delta(delta: isize) {
    if delta == 0 {
        return;
    }

    let now = if delta > 0 {
        let d = delta as usize;
        let now = METRICS.current_bytes.fetch_add(d, Ordering::Relaxed) + d;
        update_peak(now);
        now
    } else {
        let d = (-delta) as usize;
        let old = METRICS.current_bytes.fetch_sub(d, Ordering::Relaxed);
        old - d
    };

    tls_on_bytes_change(now);
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        METRICS.alloc_calls.fetch_add(1, Ordering::Relaxed);

        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            apply_delta(layout.size() as isize);
        }
        ptr
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        METRICS.alloc_calls.fetch_add(1, Ordering::Relaxed);

        let ptr = unsafe { System.alloc_zeroed(layout) };
        if !ptr.is_null() {
            apply_delta(layout.size() as isize);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        METRICS.dealloc_calls.fetch_add(1, Ordering::Relaxed);

        unsafe { System.dealloc(ptr, layout) };
        apply_delta(-(layout.size() as isize));
    }

    unsafe fn realloc(&self, ptr: *mut u8, old_layout: Layout, new_size: usize) -> *mut u8 {
        METRICS.realloc_calls.fetch_add(1, Ordering::Relaxed);

        let new_ptr = unsafe { System.realloc(ptr, old_layout, new_size) };
        if !new_ptr.is_null() {
            let old = old_layout.size();
            let delta = new_size as isize - old as isize;
            apply_delta(delta);
        }
        new_ptr
    }
}

pub struct ProfileScope {
    t0: Instant,
    label: &'static str,

    base_alloc: usize,
    base_dealloc: usize,
    base_realloc: usize,

    prev_tls: ScopeState,
}

impl ProfileScope {
    pub fn new(label: &'static str) -> Self {
        let base_current = METRICS.current_bytes.load(Ordering::Relaxed);

        let base_alloc = METRICS.alloc_calls.load(Ordering::Relaxed);
        let base_dealloc = METRICS.dealloc_calls.load(Ordering::Relaxed);
        let base_realloc = METRICS.realloc_calls.load(Ordering::Relaxed);

        let prev_tls = TLS_SCOPE.with(|tls| {
            let prev = tls.get();
            tls.set(ScopeState {
                active: true,
                base: base_current,
                peak_delta: 0,
            });
            prev
        });

        Self {
            t0: Instant::now(),
            label,
            base_alloc,
            base_dealloc,
            base_realloc,
            prev_tls,
        }
    }
}

impl Drop for ProfileScope {
    fn drop(&mut self) {
        let elapsed = self.t0.elapsed();

        let peak_delta = TLS_SCOPE.with(|tls| tls.get().peak_delta);

        let alloc = METRICS.alloc_calls.load(Ordering::Relaxed) - self.base_alloc;
        let dealloc = METRICS.dealloc_calls.load(Ordering::Relaxed) - self.base_dealloc;
        let realloc = METRICS.realloc_calls.load(Ordering::Relaxed) - self.base_realloc;

        eprintln!(
            "\n--- profile ({}) ---\nTime: {:?}\nPeak delta: {}\nalloc/dealloc/realloc calls: {}/{}/{}\n",
            self.label,
            elapsed,
            format_signed_bytes(peak_delta),
            alloc,
            dealloc,
            realloc,
        );

        TLS_SCOPE.with(|tls| tls.set(self.prev_tls));
    }
}

pub fn format_bytes(b: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut unit = 0;
    let mut f = b as f64;

    while f >= 1024.0 && unit + 1 < UNITS.len() {
        f /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{b} {}", UNITS[unit])
    } else {
        format!("{:.2} {}", f, UNITS[unit])
    }
}

fn format_signed_bytes(b: isize) -> String {
    if b < 0 {
        format!("-{}", format_bytes((-b) as usize))
    } else {
        format_bytes(b as usize)
    }
}
