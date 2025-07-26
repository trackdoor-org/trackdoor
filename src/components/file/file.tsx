import './file.css';
import { Button } from "@components/index";
import closeIcon from '@assets/icons/close.svg';
import diskIcon from '@assets/icons/disk.svg';
import eyeIcon from '@assets/icons/eye.svg';
import chevronDownIcon from '@assets/icons/chevron-down-small.svg';
import chevronRightIcon from '@assets/icons/chevron-right-small.svg';
import { GpxFile } from '@/types/types';


interface FileProps {
  gpxFile: GpxFile;
}


function File({ gpxFile }: FileProps) {
  return (
    <div className="file">
      
      <div className="header">

        <div className="left">
          <img className="collapse" src={ chevronRightIcon }></img>
          <label>{gpxFile.name}</label>
          <Button icon={ eyeIcon }/>
        </div>

        <div className="right">
          <Button icon={ diskIcon }/>
          <Button icon={ closeIcon }/>
        </div>

      </div>
    
    </div>
  );
}

export default File;
