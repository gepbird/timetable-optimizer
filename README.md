# Timetable optimizer

## Goal

The goal of this project finding the best timetable by generating all the possible combinations of timetables and you iteratively filter them until you find the best.
You have to input the subject you want to take, and timetables with courses will be generated.

## Non goals

- give you a single best timetable with minimal filtering
- select subjects for you

## Example usage

```sh
$ cargo run
# empty filter will return all the timetables
Enter filter:
Filtered timetables: 144
Enter filter: free_workday=
Filtered timetables: 32
Enter filter: max_gap_minutes_between_courses=150
Filtered timetables: 40
# multiple filters
Enter filter: free_workday= max_gap_minutes_between_courses=150
Filtered timetables: 12
```

## Filters

### min_start_time

Passes when there are no earlier courses on any day than the specified time.
Format: hours::minutes
Example: min_start_time=8:00

### max_end_time

Passes when there are no later courses on any day than the specified time.
Format: hours::minutes
Example: max_end_time=16:00

### free_workday

Passes when there are no courses on at least one workday (Monday to Friday).
Format: none
Example: free_weekday=

### max_gap_minutes_between_courses

Passes when there are no two consecutive courses on any day, that have more minutes between them than the specified value.
Format: integer
Example: max_gap_minutes_between_courses=30

### exclude_teacher

Passes when none of the courses' teachers' names includes the specified name.
Format: string
Example: exclude_teacher=John
Example: exclude_teacher=Jo

### no_course_between

Passes when there are no courses between the specified times on the specified weekday.
Format weekday,hours:minutes,hours:minutes
Example: no_course_between=wed,9:30,11:50
Example: no_course_between=monday,12:00,15:00

## Workflow

My experimental workflow for this project is starting with a CLI app with the subject information hard-coded, and make an advanced version every time.
The final app should be a web app, gathering subject data from Neptun and a performant live updating filterer, most likely full client-side.
Check out [TODO](TODO.md) to track this project's progess.
