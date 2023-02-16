
pub fn print_instructions() {
    println!("\nWelcome to Hunter's Doomsday Algorithm Checker\n");

    println!("Doomsday Dates are as follows:\n\t1/3-4\t2/28-29\t  3/14\n\t4/4\t5/9\t  6/6\n\t7/11\t8/8\t  9/5\n\t10/10\t11/7\t  12/12\n");

    println!("Weekdays can be represented as numbers like so:\n\tSunday = 0\n\tMonday = 1\n\tTuesday = 2\n\tWednesday = 3\n\tThursday = 4\n\tFriday = 5\n\tSaturday = 6\n");

    println!("Doomsday for 2000 is Tuesday (2), Doomsday for 1900 is Wednesday (3),\nDoomsday for 1800 is Friday (5), Doomsday for 1700 is Sunday (0)\nThis pattern repeats forwards and backwards for all centuries\n");

    println!("Special years:\n\t12 = 1\t24 = 2\t36 = 3\t48 = 4\n\t60 = 5\t72 = 6\t84 = 7\t96 = 8\n");

    println!("Basic Calculation is: (century + special year + years to guess year + (guess date - closest doomsday date)) % 7 = weekday number\n");
}
