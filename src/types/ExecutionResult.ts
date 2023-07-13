import { Student } from "./Student";

export type ExecutionResult = {
    new_seat_assignment: (Student | null)[][];
    score: number;
}