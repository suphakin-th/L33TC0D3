class Solution:
    def expand_around_center(self, s: str, left: int, right: int) -> str:
        while left >= 0 and right < len(s) and s[left] == s[right]:
            left -= 1
            right += 1
        return s[left + 1:right]

    def longestPalindrome(self, s: str) -> str:
        if len(s) <= 1:
            return s
        longest = s[0]
        for i in range(len(s)):
            odd_palindrome = self.expand_around_center(s, i, i)
            even_palindrome = self.expand_around_center(s, i, i + 1)
            current_longest = odd_palindrome if len(odd_palindrome) > len(even_palindrome) else even_palindrome
            if len(current_longest) > len(longest):
                longest = current_longest
        return longest