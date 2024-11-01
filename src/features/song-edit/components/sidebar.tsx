import { Sidebar as ShadcnSidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider } from "@/components/ui/sidebar"
import { useParams, useLocation, NavLink, useNavigate } from "react-router-dom"
import { TbMetronome } from "react-icons/tb";
import { FaList } from "react-icons/fa";
import { MdChevronLeft, MdOutlinePiano } from "react-icons/md";
import { FaGuitar } from "react-icons/fa6";
import { useAtomValue } from "jotai";
import { songEditAtom } from "../atoms/song-edit";
import { twMerge } from "tailwind-merge";

export const Sidebar = () => {
    const { uuid } = useParams()
    const { pathname } = useLocation()
    const song = useAtomValue(songEditAtom)
    const navigate = useNavigate()

    // Menu items.
    const items = [
        {
            title: "song details",
            url: `/song-edit/${uuid}/details`,
            icon: FaList,
        },
        {
            title: "timing editor",
            url: `/song-edit/${uuid}/timing-editor`,
            icon: TbMetronome,
            disabled: !song?.title
        },
        {
            title: "midi editor",
            url: `/song-edit/${uuid}/midi-editor`,
            icon: MdOutlinePiano,
            disabled: !song?.title
        },
        {
            title: "game view",
            url: `/song-edit/${uuid}/game-view`,
            icon: FaGuitar,
            disabled: !song?.title
        },
    ]

    return (
        <ShadcnSidebar collapsible="icon">
            <SidebarContent>
                <SidebarGroup>
                <SidebarGroupLabel>song editor</SidebarGroupLabel>
                <SidebarGroupContent>
                    <SidebarMenu>
                    {items.map((item) => (
                        <SidebarMenuItem key={item.title}>
                            <SidebarMenuButton isActive={pathname === item.url} asChild onClick={() => {
                                if (item.disabled) return
                                navigate(item.url)
                            }} className={twMerge(
                                item.disabled ? "text-neutral-600 cursor-not-allowed" : ""
                            )}>
                                <button>
                                    <item.icon />
                                    <span>{item.title}</span>
                                </button>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    ))}
                    </SidebarMenu>
                </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
            <SidebarFooter>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton asChild>
                            <NavLink to={"/song-select"}>
                                <MdChevronLeft />
                                <span>save and exit</span>
                            </NavLink>
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarFooter>
        </ShadcnSidebar>
    )
}
