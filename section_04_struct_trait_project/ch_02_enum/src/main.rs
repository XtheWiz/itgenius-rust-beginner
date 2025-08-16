#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// ตัวอย่างการใช้งานจริง: ระบบจัดการสถานะงาน
#[derive(Debug)]
enum TaskStatus {
    Pending,
    InProgress { assigned_to: String },
    Completed { completed_by: String, duration: u32 },
    Cancelled { reason: String },
}

#[derive(Debug)]
struct Task {
    id: u32,
    title: String,
    status: TaskStatus,
}

fn main() {
    // การสร้างตัวอย่าง Task
    let task1 = Task {
        id: 1,
        title: String::from("Implement feature X"),
        status: TaskStatus::Pending,
    };
    let task2 = Task {
        id: 2,
        title: String::from("Fix bug Y"),
        status: TaskStatus::InProgress {
            assigned_to: String::from("Alice"),
        },
    };
    let task3 = Task {
        id: 3,
        title: String::from("Review PR Z"),
        status: TaskStatus::Completed {
            completed_by: String::from("Bob"),
            duration: 120,
        },
    };
    let task4 = Task {
        id: 4,
        title: String::from("Update documentation"),
        status: TaskStatus::Cancelled {
            reason: String::from("Outdated information"),
        },
    };

    // การพิมพ์ค่าของ Task
    println!("Task 1: {:?}", task1);
    println!("Task 2: {:?}", task2);
    println!("Task 3: {:?}", task3);
    println!("Task 4: {:?}", task4);
}

fn main_enum() {
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;

    println!("IPV4: {:?}", ipv4);
    println!("IPV6: {:?}", ipv6);

    // การสร้างตัวแปรจาก Enum IpAddr
    let localhost = IpAddr::V4(127, 0, 0, 1);
    let google = IpAddr::V6(String::from("::1"));
    // การพิมพ์ค่าของตัวแปรที่เป็น Enum
    println!("Localhost IP: {:?}", localhost);
    println!("Google IP: {:?}", google);

    if let IpAddr::V4(a, b, c, d) = localhost {
        println!("Localhost IP: {}.{}.{}.{}", a, b, c, d);
    }
}
