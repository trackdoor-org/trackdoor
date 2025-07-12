import "./sidebar.css";


interface SidebarProps {
  isCollapsed?: boolean;
}


function Sidebar({ isCollapsed = false }: SidebarProps) {

  if (!isCollapsed) {
    return (
     <div className="sidebar">
       <p>Item 1</p>
       <p>Item 2</p>
     </div>
   )
  }
}

export default Sidebar;
