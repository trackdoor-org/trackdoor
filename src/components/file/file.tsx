import './file.css';
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@components/index";
import closeIcon from '@assets/icons/close.svg';
import eyeIcon from '@assets/icons/eye.svg';
import chevronDownIcon from '@assets/icons/chevron-down-small.svg';
import chevronRightIcon from '@assets/icons/chevron-right-small.svg';
import { GpxFile } from '@/types/types';


interface FileProps {
  gpxFile: GpxFile;
  index: number;
}


function File({ gpxFile, index }: FileProps) {
  const [isCollapsed, setIsCollapsed] = useState(true)

  const closeGpxFile = async ()=> {
    await invoke('close_gpx_file', { index: index });

  };

  return (
    <div className="file">
      
      <div className="header">
        <div className="left">
          <img className="collapse" src={ isCollapsed ? chevronRightIcon : chevronDownIcon }
            onClick={ ()=> {setIsCollapsed(!isCollapsed)} }>
          </img>
          <label>{gpxFile.is_saved ?  gpxFile.name: gpxFile.name + "*"}</label>
        </div>

        <div className="right">
          <Button icon={ eyeIcon }/>
          <Button icon={ closeIcon } onClick={ closeGpxFile }/>
        </div>
      </div>

    </div>
  );
}

export default File;
