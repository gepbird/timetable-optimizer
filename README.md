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
