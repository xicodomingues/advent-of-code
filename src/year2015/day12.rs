use regex::Regex;
use serde_json::Value;

pub fn find_nums(val: &Value) -> i64 {
    match val {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(x) => x.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(a) => a.iter().map(find_nums).sum(),
        Value::Object(obj) => {
            if obj.iter().any(|(_, v)| v.as_str() == Some("red")) {
                0
            } else {
                obj.iter().map(|(_, v)| find_nums(v)).sum()
            }
        },
    }
}

pub fn part1(input: &str) -> i32 {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(input).map(|m| m.as_str().parse::<i32>().unwrap()).sum()
}

pub fn part2(input: &str) -> i64 {
    let v: Value = serde_json::from_str(input).unwrap();
    find_nums(&v)
}

#[test]
fn test() {
    assert_eq!(part1("[1,2,3]"), 6);
    assert_eq!(part1(r#"{"a":2,"b":4}"#), 6);
    assert_eq!(part1("[[[3]]]"), 3);
    assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#), 3);
    assert_eq!(part1(r#"{"a":[-1,1]}"#), 0);
    assert_eq!(part1(r#"[-1,{"a":1}]"#), 0);
    
    assert_eq!(part2("[1,2,3]"), 6);
    assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#), 4);
    assert_eq!(part2(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
    assert_eq!(part2(r#"[1,"red",5]"#), 6);
}