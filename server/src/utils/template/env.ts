export const env = (key: string, default_value?: unknown) => {
    return process.env[key] || default_value || null
}
