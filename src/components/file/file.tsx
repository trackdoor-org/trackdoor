import "./file.css";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@components/index";
import closeIcon from "@assets/icons/close.svg";
import eyeIcon from "@assets/icons/eye.svg";
import saveIcon from "@assets/icons/disk.svg";
import chevronDownIcon from "@assets/icons/chevron-down-small.svg";
import chevronRightIcon from "@assets/icons/chevron-right-small.svg";
import { GpxFile } from "@/types/types";

interface FileProps {
  gpxFile: GpxFile;
  index: number;
  selectedFileIdx: number;
}

function File({ gpxFile, index, selectedFileIdx }: FileProps) {
  const [isCollapsed, setIsCollapsed] = useState(true);

  const closeGpxFile = async () => {
    await invoke("close_gpx_file", { index: index });
  };

  const saveGpxFile = async () => {
    await invoke("save_gpx_file", { index: index });
  };

  const selectGpxFile = async () => {
    await invoke("select_gpx_file", { index: index });
  };

  return (
    <div
      className={"file" + (index == selectedFileIdx ? " selected" : "")}
      onClick={() => {
        if (index != selectedFileIdx) {
          selectGpxFile();
        }
      }}
    >
      <div className="header">
        <div className="left">
          <img
            className="collapse"
            src={isCollapsed ? chevronRightIcon : chevronDownIcon}
            onClick={() => {
              setIsCollapsed(!isCollapsed);
            }}
          ></img>
          <label>{gpxFile.isSaved ? gpxFile.name : gpxFile.name + "*"}</label>
        </div>
        <div className="right">
          <Button icon={saveIcon} onClick={saveGpxFile} />
          <Button icon={eyeIcon} />
          <Button icon={closeIcon} onClick={closeGpxFile} />
        </div>
      </div>
    </div>
  );
}

export default File;
