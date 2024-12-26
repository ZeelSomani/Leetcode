class Solution {
    function isPalindrome($x) {
        $str = (string) $x;
        $str_reverse = strrev($str);
        return $str === $str_reverse;
    }
}
