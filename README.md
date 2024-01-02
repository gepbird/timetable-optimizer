# Timetable optimizer

## Goal

The goal of this project finding the best timetable by generating all the possible combinations of timetables and you iteratively filter them until you find the best.
You have to input the subject you want to take, and timetables with courses will be generated.

## Non goals:

- give you a single best timetable with minimal filtering
- select subjects for you

## Workflow

My experimental workflow for this project is starting with a CLI app with the subject information hard-coded, and make an advanced version every time.
The final app should be a web app, gathering subject data from Neptun and a performant live updating filterer, most likely full client-side.
Check out [TODO](TODO.md) to track this project's progess.

## Example usage

```sh
$ cargo run
# empty filter will return all the timetables
Enter filter:
Filtered timetables: 144
Enter filter: excluded_weekday=thu
Filtered timetables: 32
Enter filter: max_gap_minutes_between_courses=60
Filtered timetables: 72
# multiple criterias
Enter filter: excluded_weekday=thu max_gap_minutes_between_courses=60
Filtered timetables: 16
```

## Filters

### min_start_time

Passes when there are no earlier courses on any day than the specified time.

Format: hours::minutes

Example: 8:00

### max_end_time

Passes when there are no later courses on any day than the specified time.

Format: hours::minutes

Example: 16:00

### excluded_weekday

Passess when there are no courses on the specified weekday.

Format: chrono Weekday

Example: monday

Example: wed

### max_gap_minutes_between_courses

Passess when there are no two consecutive courses on any day, that have more minutes between them than the specified value.

Format: integer

Example: 30
