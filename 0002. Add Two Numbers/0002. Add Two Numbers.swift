
class Solution {
    func addTwoNumbers(_ l1: ListNode?, _ l2: ListNode?) -> ListNode? {
        var dummy = ListNode.init()
        var carry = 0
        var l1 = l1
        var l2 = l2
        var cur = dummy
        while l1 != nil || l2 != nil || carry != 0 {
            let s = (l1?.val ?? 0) + (l2?.val ?? 0) + carry
            carry = s / 10
            cur.next = ListNode.init(s % 10)
            cur = cur.next!
            l1 = l1?.next
            l2 = l2?.next
        }
        return dummy.next
    }
}
