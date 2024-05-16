class Solution:
    def myAtoi(self, s: str) -> int:
        INT_MAX = 2**31 - 1
        INT_MIN = -2**31
        pattern = re.compile(r'^\s*([+-]?\d+)')
        match = pattern.match(s)
        if not match:
            return 0
        num_str = match.group(1)
        num = int(num_str)
        if num < INT_MIN:
            return INT_MIN
        if num > INT_MAX:
            return INT_MAX
        return num