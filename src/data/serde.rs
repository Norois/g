pub trait ToJsonString{
    fn to_json_string(&self) -> String;
}

impl ToJsonString for crate::map::Map{
    fn to_json_string(&self) -> String {
        self.map.to_json_string()
    }
}
impl ToJsonString for crate::map::Tile{
    fn to_json_string(&self) -> String {
        format!("{{\"material\": {}, \"can_walk_on\": {}}}",
        self.material.to_json_string(),
        self.can_walk_on.to_json_string())
    }
}
impl ToJsonString for crate::Material{
    fn to_json_string(&self) -> String {
        format!("{{\"character\": {}, \"color\": {}}}",
        self.character.to_json_string(), 
        (self.color.0, self.color.1, self.color.2).to_json_string())
    }
}
impl ToJsonString for crate::Position{
    fn to_json_string(&self) -> String {
        format!("[{},{}]", self.x, self.y)
    }
}

impl ToJsonString for u8{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for i8{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for u16{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for i16{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for u32{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for i32{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for u64{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for i64{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for u128{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for i128{
    fn to_json_string(&self) -> String {
        self.to_string()
    }
}
impl ToJsonString for char{
    fn to_json_string(&self) -> String {
        format!("\"{}\"", self)
    }
}
impl ToJsonString for String{
    fn to_json_string(&self) -> String {
        format!("\"{}\"", self)
    }
}
impl ToJsonString for &str{
    fn to_json_string(&self) -> String {
        format!("\"{}\"", self)
    }
}
impl ToJsonString for bool{
    fn to_json_string(&self) -> String {
        if *self{
            return String::from("true");
        }
        String::from("false")
    }
}
impl<T: ToJsonString> ToJsonString for Vec<T>{
    fn to_json_string(&self) -> String {
        let final_string: Vec<String> = self.iter().map(|v| v.to_json_string()).collect();
        format!("[{}]", final_string.join(","))
    }
}
impl<T: ToJsonString, const N: usize> ToJsonString for [T; N] {
    fn to_json_string(&self) -> String {
        let mut final_string: String = String::new();
        final_string.push('[');
        for i in self{
            final_string.push_str(&format!("{},", i.to_json_string()));
        }
        final_string.pop();
        final_string.push(']');
        final_string
    }
}
impl<T: ToJsonString, Y: ToJsonString> ToJsonString for std::collections::hash_map::HashMap<T, Y>{
    fn to_json_string(&self) -> String {
        let mut final_string: String = String::new();
        final_string.push('{');
        for i in self{
            final_string.push_str(&format!("\"{}\": {},", i.0.to_json_string().replace('\"', ""), i.1.to_json_string()));
        }
        final_string.pop();
        final_string.push('}');
        final_string
    }
}
impl<T: ToJsonString, Y: ToJsonString, I: ToJsonString> ToJsonString for (T, Y, I){
    fn to_json_string(&self) -> String {
        format!("[{},{},{}]", self.0.to_json_string(), self.1.to_json_string(), self.2.to_json_string())
    }
}