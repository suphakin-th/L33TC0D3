class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        n = (nums1+nums2)
        n.sort()
        rang = len(n)
        if rang % 2 == 1:
            return float(n[rang // 2])
        else:
            return (n[rang//2-1] + n[rang//2])/2.0
        