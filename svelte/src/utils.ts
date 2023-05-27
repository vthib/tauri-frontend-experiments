export function formatWithUnits(qtty: number): string {
    if (qtty < 1024) {
        return `${qtty}`;
    }
    qtty /= 1024;

    if (qtty < 1024) {
        return `${Math.round(qtty)} KB`;
    }
    qtty /= 1024;

    if (qtty < 1024) {
        return `${Math.round(qtty)} MB`;
    }
    qtty /= 1024;

    return `${Math.round(qtty)} GB`;
}
