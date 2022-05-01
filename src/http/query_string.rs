use std::collections::HashMap;

pub struct QueyString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>)
}

impl<'buf> QueyString<'buf> {
    pub fn getValue(&self, key: &str) -> Option<&Value>{
        self.data.get(key)
    }
}

// a=1&b=11&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueyString<'buf>{
    
    fn from(s: &'buf str) -> Self{
        let mut data = HashMap::new();

        for sub_str in s.split('&'){
            let mut key = sub_str;
            let mut val = "";
            if let Some(i) = sub_str.find('=') {
                key= &sub_str[..i];
                val = &sub_str[i+1..];
            }

            data.entry(key)
            .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // let mut vec = Vec::new();
                        // vec.push(val);
                        // vec.push(prev_val); 
                        
                        //let mut vec = vec![prev_val, val];
                        *existing = Value::Multiple(vec![prev_val,val]);
                    }
                    Value::Multiple(vec) => vec.push(val)
                
            })
            .or_insert(Value::Single(val));
        
        }    
        QueyString {data};

        unimplemented!()
    }
    
}