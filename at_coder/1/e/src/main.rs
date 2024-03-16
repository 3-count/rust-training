use std::io;

#[derive(Clone)]
struct MemberId {
    value: i32
}

impl MemberId {
    fn new(value: i32) -> Self {
        return MemberId { value }
    }

    fn equals(&self, other: &MemberId) -> bool {
        return self.value == other.value;
    }
}

struct Member {
    id: MemberId,
    followers: Vec<*mut Member>,
    followees: Vec<*mut Member>
}

impl Member {
    fn new(id: i32) -> Member {
        return Member { id: MemberId::new(id), followers: Vec::new(), followees: Vec::new() }
    }

    fn id(&self) -> &MemberId {
        return &self.id;
    }

    fn equals(&self, member: &Member) -> bool {
        return self.id().equals(member.id())
    }

    fn follows(&self, member: &Member) -> bool {
        return self.followees.iter().any(|f| unsafe {
            return f.as_mut().unwrap().equals(member);
        });
    }

    fn follow(&mut self, member: *mut Member) {
        unsafe {
            if self.follows(&*member) || self.equals(&*member)  {
                return;
            }
        }

        self.followees.push(member);
        unsafe {
            (*member).followers.push(self);
        }
    }

    fn follow_all_followers(&mut self) {
        let member = unsafe{ (self as *mut Self).as_ref().unwrap() };
        self.follow_all_followers_of(member);
    }

    fn follow_all_followees_of_followees(&mut self) {
        let member = unsafe {(self as *mut Self).as_mut().unwrap()};
        for i in 0..self.followees.len() {
            unsafe {
                let followee = self.followees.get_mut(i).unwrap().as_mut().unwrap();
                followee.followees.iter().for_each(|f| {
                    member.follow(f.as_mut().unwrap());
                });
            }
        }
    }

    fn follow_all_followers_of(&mut self, member: &Member) {
        member.followers.iter().for_each(|f| self.follow(f.cast()));
    }

    fn to_follow_result(&self, member: &Member) -> &str {
        if self.follows(member) {
            return "Y";
        }

        return "N";
    }

    fn to_relation(&self, members: &Vec<*mut Member>) -> String {
        return members.iter().map(|m| unsafe{self.to_follow_result(m.as_ref().unwrap())}).collect::<Vec<_>>().join("");
    }
}

fn analyze(log: String, members: &Vec<*mut Member>) {
    let parameters: Vec<i32> = log.split_whitespace().map(|p| p.parse::<i32>().unwrap()).collect();
    let member_id: MemberId = MemberId::new(parameters[1]);
    let member = members.iter().find(|m| unsafe{m.as_ref().unwrap().id().equals(&member_id)}).unwrap();
    match parameters[0] {
        1 => {
            let follower_id: MemberId = MemberId::new(parameters[2]);
            let follower = members.iter().find(|m| unsafe {m.as_ref().unwrap().id().equals(&follower_id)}).unwrap();
            unsafe {
                member.as_mut().unwrap().follow(follower.as_mut().unwrap());
            }
        },
        2 => unsafe {member.as_mut().unwrap().follow_all_followers()},
        3 => unsafe {member.as_mut().unwrap().follow_all_followees_of_followees()},
        _ => {}
    }
}

fn main() {    
    let mut line: String = String::new();
    io::stdin().read_line(&mut line).ok();
    let parameters: Vec<i32> = line.trim().replace("\r", "").split_whitespace().map(
        |p| p.parse::<i32>().unwrap()).collect();
    let member_count: i32 = parameters[0];
    let log_count: i32 = parameters[1];
    let mut members: Vec<Member> = Vec::with_capacity(member_count as usize);
    for i in 1..member_count + 1 {
        members.push(Member::new(i));
    }

    let member_pointers = members.iter_mut().map(|f| f as *mut Member).collect::<Vec<*mut Member>>();
    for _ in 0..log_count {
        line = String::new();
        io::stdin().read_line(&mut line).ok();
        analyze(line, &member_pointers);
    }

    member_pointers.iter().for_each(|m| println!("{}", unsafe{m.as_ref().unwrap().to_relation(&member_pointers)}));
}