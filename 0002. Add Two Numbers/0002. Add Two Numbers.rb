
def add_two_numbers(l1, l2)
    dummy = ListNode.new()
    carry = 0
    cur = dummy
    while !l1.nil? || !l2.nil? || carry > 0
        s = (l1.nil? ? 0 : l1.val) + (l2.nil? ? 0 : l2.val) + carry
        carry = s / 10
        cur.next = ListNode.new(s % 10)
        cur = cur.next
        l1 = l1.nil? ? l1 : l1.next
        l2 = l2.nil? ? l2 : l2.next
    end
    dummy.next
end
