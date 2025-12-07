import { main } from ".";

describe("07", () => {
    describe("07", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(21);
        });
    });

    describe("07", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(40);
        });
    });
});
