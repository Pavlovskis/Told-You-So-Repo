#![allow(dead_code)]
pub mod url{
    use std::collections::HashMap;

    pub fn encode(url:String) -> String {
        let percent_map:HashMap<char, &str> = HashMap::from([
            (' ', "%20"),('!', "%21"),('"', "%22"),('#', "%23"),('$', "%24"),('%', "%25"),('&', "%26"),('\'', "%27"),('(', "%28"),(')', "%29"),
            ('*', "%2A"),('+', "%2B"),(',', "%2C"),('-', "%2D"),('.', "%2E"),('/', "%2F"),(':', "%3A"),(';', "%3B"),('<', "%3C"),('=', "%3D"),
            ('>', "%3E"),('?', "%3F"),('@', "%40"),('[', "%5B"),(']', "%5D"),('_', "%5F")
        ]);

        let v:Vec<char> = url.chars().collect();

        let mut encoded_url:String = String::new();

        for c in 0..v.len() {
            if percent_map.contains_key(&v[c]){
                match percent_map.get(&v[c]) {
                    Some(p) => {
                        encoded_url.push_str(p);
                    },
                    None => {
                        continue;
                    }
                }
            }else {
                encoded_url.push(v[c]);
            }
        }

        encoded_url
    }

    pub fn decode(url:String) -> String{
        let percent_map:HashMap<&str, char> = HashMap::from([
            ("%20", ' '),("%21", '!'),("%22", '"'),("%23", '#'),("%24", '$'),("%25", '%'),("%26", '&'),("%27", '\''),("%28", '('),("%29", ')'),
            ("%2A",'*'),("%2B",'+'),("%2C",','),("%2D",'-'),("%2E",'.'),("%2F",'/'),("%3A",':'),("%3B",';'),("%3C",'<'),("%3D",'='),("%3E",'>'),
            ("%3F",'?'),("%40",'@'),("%5B",'['),("%5D",']'),("%5F",'_')
        ]);

        let v:Vec<char> = url.chars().collect();

        let mut decoded_url:String = String::new();

        let mut c:usize = 0;
        while c < v.len() {
            if v[c] == '%' {                
                let perc_char:Vec<char> = v[c..c + 3].to_vec();
                let perc:String = perc_char.into_iter().collect();
                if percent_map.contains_key(perc.as_str()){
                    match percent_map.get(perc.as_str()) {
                        Some(p) => {
                            decoded_url.push(*p);
                        }
                        None => {
                            continue;
                        }
                    }
                }
                c += 3;
            }else {
                decoded_url.push(v[c]);
                c += 1;
            }
        }

        decoded_url
    }
}

pub mod base64 {
    pub fn encode(code:String) {
        
    }
}

pub mod encrypt {
    pub fn shift(pass:String, n:usize) -> String{
        let shifts:usize = n % pass.len();
        let chars:Vec<char> = pass.chars().collect();
    
        let mut new:String = String::with_capacity(pass.len());
        for i in pass.len() - shifts..pass.len() { 
            new.push(chars[i]);
        }
        for i in 0..pass.len() - shifts {
            new.push(chars[i]);
        }
    
        new
    }
    
    pub fn hashme(s:String) -> String {
        use sha2::{Sha512, Digest};
    
        let mut hasher:Sha512 = Sha512::new();
    
        hasher.update(s);
    
        let result = hasher.finalize();
    
        hex::encode(result)
    }
    
    pub fn salt_password(strng:String, mut pass:String) -> String{
        let hash:String = hashme(strng);
    
        let salt:String = hash[5..10].to_string();
    
        let mut salted_password:String = String::new();
        for i in pass.char_indices() {
            if i.0 == 4 {
                salted_password.push_str(&salt);
            }
            salted_password.push(i.1);
        }
        pass = salted_password;
    
        pass
    }
    
    pub fn encrypt_password(user:User) {
        
        let mut salted:String = String::new();
        if let Some(name) = user.name {
            salted = salt_password(name, user.password);
        
        }else if let Some(mail) = user.mail {
            salted = salt_password(mail, user.password);
        
        }else {
            panic!("No Credentials were found");
        }
    
        let shifted:String = shift(salted, user.id as usize);
    
        let hashed:String = hashme(shifted);
    
        println!("{}", hashed);
    }
    
    // pub fn compress(chars: &mut Vec<char>) -> i32{
    //     let mut count:usize = 0;
    //     let first_char:char = chars[0];
    
    //     for c in chars {
    //         if c != &first_char {
    //             chars.push('c');
    //         }
    //         count += 1;
    //     }
    
    //     chars.len() as i32
    // }
}
