import py_type_conversion_traits as tc
from datetime import date


def main() -> None:
    # Example 1 - Add two numbers and print the result and its type.

    result = tc.add(1, 2)
    print(result, type(result))

    # Example 2 - Pass a list to the extension, and get a list back.

    nums = [1.0, 2.0, 3.0, 4.0, 5.0]
    double_nums = tc.map_float(lambda x: x * 2, nums)
    print(double_nums, type(double_nums))

    # Example 3 - Pass a dictionary to the extension, and get a tuple
    # of lists back.

    colors = {
        "red": "ff0000",
        "green": "00ff00",
        "blue": "0000ff",
        "yellow": "ffff00",
        "black": "000000",
        "white": "ffffff",
    }
    color_names, color_hexcodes = tc.extract_strings_from_dict(colors)
    print(color_names, color_hexcodes)

    # Example 4 - Work with Date objects.

    first_date = date(2021, 1, 1)
    last_date = date(2021, 1, 31)

    days_between = tc.get_weekdays(first_date, last_date)
    print(days_between, type(days_between))

    # Example 5 - Work with Point tuple.

    point_a = tc.Point(1, 2)
    point_b = tc.Point(3, 4)
    d = point_a.distance(point_b)
    print(d, type(d))

    # Example 6 - Custom Person Object

    person = tc.Person("Bob", "Smith")
    print(person.full_name())


if __name__ == "__main__":
    main()
