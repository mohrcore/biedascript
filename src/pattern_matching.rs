//use ::std::hash;
//use ::std::collections::hash_map::DefaultHasher;


pub fn pmatch_str<'a>(s: &'a str, pattern: &[&str]) -> Option<Vec<&'a str>> {

    let mut res = Vec::<&'a str>::new();
    let mut p_vec = Vec::<usize>::new();
    
    let c_idx = 0;

    for pat in pattern {
        let idc = s.match_indices(pat);
        for (i, _) in s.match_indices(pat) {
            if i >= c_idx {
                p_vec.push(i);
                c_idx = i;
            } else {
                return None;
            }
        }
    }

    //TODO: FINISH

    Some(res)
}