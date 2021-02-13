export const generatePathQuery = (path :string, obj:object) =>
    path +
    Object.entries(obj)
        .reduce((total, [k, v]) => (total += `${k}=${encodeURIComponent(v)}&`), "?")
        .slice(0, -1);
