import { main } from ".";

describe("05", () => {
    describe("01", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(143);
        });
    });

    describe("02", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(123);
        });
    });
});
