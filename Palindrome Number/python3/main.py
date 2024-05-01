class Solution:
    def isPalindrome(self, x: int) -> bool:
        str_x = str(x)
        length = len(str_x)
        for i in range(length // 2):
            if str_x[i] != str_x[length - i - 1]:
                return False
        return True
        