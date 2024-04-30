class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        if not s:
            return 0
        max_length = 0
        substring = ''
        for char in s:
            if char not in substring:
                substring += char
                max_length = max(max_length, len(substring))
            else:
                index = substring.index(char)
                print(index, substring)
                substring = substring[index + 1:] + char
                print(substring)
        return max_length