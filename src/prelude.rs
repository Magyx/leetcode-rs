use serde_json::Value;

/// Definition for singly-linked list (LeetCode-compatible).
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

pub fn list_node_from_value(v: &Value) -> Result<Option<Box<ListNode>>, String> {
    match v {
        Value::Null => Ok(None),
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok(None);
            }

            let mut head: Option<Box<ListNode>> = None;
            // Build in reverse so it is O(n)
            for item in arr.iter().rev() {
                let n = item
                    .as_i64()
                    .ok_or_else(|| format!("expected integer, got {item}"))?;
                if n < i32::MIN as i64 || n > i32::MAX as i64 {
                    return Err(format!("integer out of i32 range: {n}"));
                }
                let mut node = Box::new(ListNode::new(n as i32));
                node.next = head;
                head = Some(node);
            }
            Ok(head)
        }
        other => Err(format!("expected null or array like [1,2,3], got {other}")),
    }
}

pub fn list_node_to_value(list: &Option<Box<ListNode>>) -> Value {
    let mut out: Vec<Value> = Vec::new();
    let mut cur = list.as_deref();
    while let Some(node) = cur {
        out.push(Value::from(node.val));
        cur = node.next.as_deref();
    }
    Value::Array(out)
}
