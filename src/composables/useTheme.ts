export function useTheme() {
    const preferredDark = usePreferredDark();
    const htmlElement = document.querySelector('html')!;

    useDark({
        onChanged: (dark) => {
            htmlElement.setAttribute('data-theme', dark ? 'dark' : 'light');
            htmlElement.classList.toggle('dark', dark);
            htmlElement.classList.toggle('light', !dark);
        },
    });

    return {
        preferredDark,
    };
}
