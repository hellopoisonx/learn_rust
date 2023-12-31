use learn_rust::LinkedList;
fn main() {
    let mut l = LinkedList::new("H");
    l.push("hello");
    l.push("hello");
    l.push("hello");
    l.display();
    l.pop();
    l.display();
    l.pop();
    l.display();
    l.pop();
    l.display();
}
