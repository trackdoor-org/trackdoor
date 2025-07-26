import "./sidebar.css";
import { invoke } from "@tauri-apps/api/core";
import { open } from '@tauri-apps/plugin-dialog';
import { Button, File } from "@components/index"
import newFileIcon from "@assets/icons/new-file.svg";
import openFileIcon from "@assets/icons/open-file.svg";
import cogwheelIcon from "@assets/icons/cogwheel.svg";
import { GpxFile } from "@/types/types";


interface SidebarProps {
  isCollapsed?: boolean;
  gpxFiles?: GpxFile[];
}


const openFile = async ()=>{
  const filePath = await open({
    multiple: false,
    directory: false,
    filters: [
    {
      name: 'My Filter',
      extensions: ['gpx'],
    },
  ],
  });

  if (filePath) {
    invoke('open_gpx_file', { pathStr: filePath });
  }
}


const createNewFile = async ()=> {
  await invoke('create_new_file');
}


function Sidebar({ isCollapsed = false, gpxFiles = [] }: SidebarProps) {

  return (
     <div className={ isCollapsed ? "sidebar collapsed" : "sidebar"}>
       <div className="sidebarMenu">
         <Button icon={ newFileIcon } onClick={ createNewFile }/>
         <Button icon={ openFileIcon } onClick={ openFile }/>
         <Button icon={ cogwheelIcon }/>
       </div>

       <div className="sidebarFileList">
        {gpxFiles.map((gpxFile: GpxFile) => (
          <File gpxFile={gpxFile}/>
        ))}
       </div>
    </div>
  )
}

export default Sidebar;
