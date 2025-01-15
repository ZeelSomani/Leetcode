
impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut new_list = ListNode::new(0);
        let mut cur = &mut new_list;
        while list1.is_some() && list2.is_some() {
            let (l1, l2) = (list1.as_deref_mut().unwrap(), list2.as_deref_mut().unwrap());
            if l1.val < l2.val {
                let next = l1.next.take();
                cur.next = list1.take();
                list1 = next;
            } else {
                let next = l2.next.take();
                cur.next = list2.take();
                list2 = next;
            }
            cur = cur.next.as_deref_mut().unwrap();
        }
        cur.next = list1.or(list2);
        new_list.next
    }
}
