import "./sidebar.css";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { Button, File } from "@components/index"
import newFileIcon from "@assets/icons/new-file.svg";
import openFileIcon from "@assets/icons/open-file.svg";
import cogwheelIcon from "@assets/icons/cogwheel.svg";


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


const createNewFile = async ()=> {
  await invoke('create_new_file');
}


function Sidebar({ isCollapsed = false }: SidebarProps) {

  return (
     <div className={ isCollapsed ? "sidebar collapsed" : "sidebar"}>
       <div className="sidebarMenu">
         <Button icon={ newFileIcon } onClick={ createNewFile }/>
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
