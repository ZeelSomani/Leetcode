
class Solution {
   
    function addTwoNumbers($l1, $l2) {
        $dummy = new ListNode(0);
        $current = $dummy;
        $carry = 0;

        while ($l1 !== null || $l2 !== null) {
            $x = $l1 !== null ? $l1->val : 0;
            $y = $l2 !== null ? $l2->val : 0;

            $sum = $x + $y + $carry;
            $carry = (int) ($sum / 10);
            $current->next = new ListNode($sum % 10);
            $current = $current->next;

            if ($l1 !== null) {
                $l1 = $l1->next;
            }

            if ($l2 !== null) {
                $l2 = $l2->next;
            }
        }

        if ($carry > 0) {
            $current->next = new ListNode($carry);
        }

        return $dummy->next;
    }
}
