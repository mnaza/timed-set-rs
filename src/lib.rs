pub struct TimedSet<T>{
    ttl: std::time::Duration,
    set: std::collections::HashMap<T, std::time::SystemTime>
}

impl<T> TimedSet<T>
where T: std::hash::Hash + Eq{
    pub fn new(ttl: std::time::Duration) -> Self{
        Self{
            ttl,
            set: std::collections::HashMap::new()
        }
    }

    pub fn add(&mut self, val: T){
        self.set.insert(val, std::time::SystemTime::now()+self.ttl);
    }

    pub fn contains(&self, val: &T) -> bool{
        if let Some(s) = self.set.get(val){
            if std::time::SystemTime::now() < *s{
                return true;
            }
        }
        false
    }

    pub fn iter(&self) -> Iter<'_, T>{
        Iter{
            set: self.set.iter().map(|(k, v)| (k, v)).collect()
        }
    }
}

pub struct Iter<'a, T> {
    set: std::collections::HashMap<&'a T, &'a std::time::SystemTime>,
}

impl<'a, T> Iterator for Iter<'a, T>
where T: Copy + std::hash::Hash + Eq{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let keys: Vec<&T> = self.set.keys().cloned().collect();
        for k in keys{
            if let Some((v, t)) = self.set.remove_entry(&k){
                if std::time::SystemTime::now() < *t{
                    return Some(v);
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let mut ts = super::TimedSet::new(std::time::Duration::from_secs(10));
        ts.add("aaaaa");
        ts.add("bbbbb");
        ts.add("ccccc");
        ts.add("ddddd");
        println!("---- step1");
        for s in ts.iter(){
            println!("{}", s);
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
        println!("---- step2");
        for s in ts.iter(){
            println!("{}", s);
        }
        println!("---- step3");
        std::thread::sleep(std::time::Duration::from_secs(5));
        for s in ts.iter(){
            println!("{}", s);
        }
    }
}
