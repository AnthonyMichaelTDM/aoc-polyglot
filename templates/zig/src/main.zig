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
    const data = @embedFile("../../input.txt");
    const input = std.mem.spanFromBytes(data);

    var part_one_result = part_one(input);
    std.debug.print("{}\n", .{part_one_result});

    var part_two_result = part_two(input);
    std.debug.print("{}", .{part_two_result});
}

test "part 1" {
    const data = @embedFile("../../example.txt");
    const input = std.mem.spanFromBytes(data);
    try std.testing.expectEqual(part_one(input), null); // change null to the expected result
}
test "part 2" {
    const data = @embedFile("../../example.txt");
    const input = std.mem.spanFromBytes(data);
    try std.testing.expectEqual(part_two(input), null); // change null to the expected result
}

