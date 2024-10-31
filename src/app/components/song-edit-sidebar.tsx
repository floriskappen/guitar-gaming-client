import { Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent, SidebarGroupLabel, SidebarMenu, SidebarMenuButton, SidebarMenuItem, SidebarProvider } from "@/components/ui/sidebar"
import { useParams, useLocation, NavLink } from "react-router-dom"
import { TbMetronome } from "react-icons/tb";
import { FaList } from "react-icons/fa";
import { MdChevronLeft, MdOutlinePiano } from "react-icons/md";
import { FaGuitar } from "react-icons/fa6";





export const SongEditSidebar = () => {
    const { uuid } = useParams()
    const { pathname } = useLocation()

    // Menu items.
    const items = [
        {
            title: "map details",
            url: `/song-edit/${uuid}/details`,
            icon: FaList,
        },
        {
            title: "timing editor",
            url: `/song-edit/${uuid}/timing-editor`,
            icon: TbMetronome,
        },
        {
            title: "midi editor",
            url: `/song-edit/${uuid}/midi-editor`,
            icon: MdOutlinePiano,
        },
        {
            title: "game view",
            url: `/song-edit/${uuid}/game-view`,
            icon: FaGuitar,
        },
    ]

    return (
            <Sidebar collapsible="icon">
                <SidebarContent>
                    <SidebarGroup>
                    <SidebarGroupLabel>song editor</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                        {items.map((item) => (
                            <SidebarMenuItem key={item.title}>
                                <SidebarMenuButton isActive={pathname === item.url} asChild>
                                    <NavLink to={item.url}>
                                        <item.icon />
                                        <span>{item.title}</span>
                                    </NavLink>
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
            </Sidebar>
            

            
    )
}
