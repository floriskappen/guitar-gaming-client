import { ThemeProvider } from "@/providers/theme-provider"

type AppProviderProps = {
    children: React.ReactNode
}

export const AppProvider = ({ children }: AppProviderProps) => {
    return (
        <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
           {children}
        </ThemeProvider>
    )
}
