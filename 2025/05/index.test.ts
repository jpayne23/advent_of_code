import { main } from ".";

describe("05", () => {
    describe("05", () => {
        it("should work", async () => {
            const result = await main("1", true);
            expect(result).toBe(3);
        });
    });

    describe("05", () => {
        it("should work", async () => {
            const result = await main("2", true);
            expect(result).toBe(14);
        });
    });
});
