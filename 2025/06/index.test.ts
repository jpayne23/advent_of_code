import { main } from ".";

describe("06", () => {
    describe("06", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(4277556);
        });
    });

    describe("06", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(3263827);
        });
    });
});
