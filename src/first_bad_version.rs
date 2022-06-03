struct Solution {
    bad_version: i32,
}
impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut beg = 1;
        let mut end = n;
        while beg <= end {
            let mid = beg + (end - beg) / 2;
            if self.isBadVersion(mid) {
                if !self.isBadVersion(mid - 1) {
                    return mid;
                } else {
                    end = mid - 1;
                }
            } else {
                if self.isBadVersion(mid + 1) {
                    return mid + 1;
                } else {
                    beg = mid + 1;
                }
            }
        }
        beg
    }
    pub fn isBadVersion(&self, version: i32) -> bool {
        return version >= self.bad_version;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    #[test]
    fn test_first_bad_version() {
        let solution = Solution { bad_version: 4 };
        assert_eq!(solution.first_bad_version(5), 4);
    }

    #[test]
    fn test_first_bad_version1() {
        let solution = Solution { bad_version: 1 };
        assert_eq!(solution.first_bad_version(1), 1);
    }
    #[test]
    fn test_first_bad_version2() {
        let solution = Solution { bad_version: 4 };
        assert_eq!(solution.first_bad_version(9), 4);
    }
    #[test]
    fn test_first_bad_versionLoop() {
        for i in 1..10000 {
            let mut rng = rand::thread_rng();
            let bad_version = rng.gen_range(1..i + 1);
            let solution = Solution { bad_version };
            assert_eq!(solution.first_bad_version(i), bad_version);
        }
    }
}
