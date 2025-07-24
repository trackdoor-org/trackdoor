import "./sidebar.css";
import Button from "../button/button.tsx"
import File from '../file/file.tsx'
import newFileIcon from "../../assets/icons/new-file.svg";
import openFileIcon from "../../assets/icons/open-file.svg";
import cogwheelIcon from "../../assets/icons/cogwheel.svg";
import { open } from '@tauri-apps/plugin-dialog';


interface SidebarProps {
  isCollapsed?: boolean;
}


const openFile = async ()=>{
  const file = await open({
    multiple: false,
    directory: false,
  });

  console.log(file);

  // TODO: Implement file opening with rust
}


function Sidebar({ isCollapsed = false }: SidebarProps) {

  return (
     <div className={ isCollapsed ? "sidebar collapsed" : "sidebar"}>
       <div className="sidebarMenu">
         <Button icon={ newFileIcon }/>
         <Button icon={ openFileIcon } onClick={ openFile }/>
         <Button icon={ cogwheelIcon }/>
       </div>

       <div className="sidebarFileList">
        <File/>
        <File/>
       </div>
    </div>
  )
}

export default Sidebar;
