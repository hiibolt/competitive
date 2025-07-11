/**
 * 71. "Simplify Path"
 * 
 * Difficulty: Medium
 * Tags: String, Stack
 * Runtime: Beats 100%
 */
impl Solution {
    pub fn simplify_path ( mut path: String ) -> String {
        path.push('/');
        let mut segments = path.split("/").collect::<Vec<&str>>();

        let mut i = 0;
        while i < segments.len() {
            if segments[i] == ".." {
                if i > 0 {
                    segments.remove(i - 1);
                    i -= 1;
                }
                segments.remove(i);
            } else if segments[i] == "" || segments[i] == "." {
                segments.remove(i);
            } else if i < segments.len() - 1 && segments[i + 1] == ".." {
                segments.remove(i);
                segments.remove(i);
            } else {
                i += 1;
            }
        }

        format!("/{}", segments.join("/"))
    }
}