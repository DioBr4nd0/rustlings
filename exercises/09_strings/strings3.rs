fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let mut i=0;
    let mut j= input.len()-1;
    while i<=j && input.as_bytes()[i]==b' '{i+=1;}
    while j>=i && input.as_bytes()[j]==b' '{j-=1;}
    &input[i..j+1]
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut out = String::from(input);
    out.push_str(" world!");
    out
}

fn replace_me(input: &str) -> String {
    let mut ans = String::new();
  let mut i =0;
    let binput = input.as_bytes();
    while i<input.len(){
        if i+3<input.len() && &input[i..i+4] =="cars"{
            ans.push_str("balloons");
            i+=4;
        }else{
            ans.push(binput[i] as char);
            i+=1;
        }
    }
    ans
 }


fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
