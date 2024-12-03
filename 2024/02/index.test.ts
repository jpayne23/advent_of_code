import { isSafe, main } from ".";

describe("01", () => {
    describe("isSafe", () => {
        it.each([
            [[7, 6, 4, 2, 1], 0, true],
            [[1, 2, 7, 8, 9], 0, false],
            [[1, 2, 7, 1, 9], 0, false],
            [[1, 2, 4, 5, 9], 1, true],
            [[1, 3, 2, 5, 4, 5], 1, false],
        ])("when input = %s, depth = %s, result = %s", (input, depth, result) => {
            const safe = isSafe(input, depth);
            expect(safe).toBe(result);
        });
    });

    describe("01", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(2);
        });
    });

    describe("02", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(4);
        });
    });
});
