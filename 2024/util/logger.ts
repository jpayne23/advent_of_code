interface LoggerInput {
    message: string;
    data: Record<string, unknown>;
}

export const logger = {
    info: (input: LoggerInput) => console.log(input),
    error: (input: LoggerInput) => console.error(input),
};
