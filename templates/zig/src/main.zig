const std = @import("std");

fn part_one(input: []const u8) ?u32 {
    // your code here
    return null;
} 

fn part_two(input: []const u8) ?u32 {
    // your code here
    return null;
}

pub fn main() void {
    const input = @embedFile("data/input.txt");

    var part_one_result = part_one(input);
    std.debug.print("{?}\n", .{part_one_result});
    std.debug.print("\n=================\n\n", .{}); // separate the parts
    var part_two_result = part_two(input);
    std.debug.print("{?}\n", .{part_two_result});
}

test "part 1" {
    const input = @embedFile("data/example.txt");
    try std.testing.expectEqual(part_one(input), null); // change null to the expected result
}
test "part 2" {
    const input = @embedFile("data/example.txt");
    try std.testing.expectEqual(part_two(input), null); // change null to the expected result
}



