import { TreeMap } from '@nivo/treemap'
import React from 'react'

const TreeMapChart = ({ data }) => {
  return (
    <div>
      <TreeMap
        data={data}
        height={100}
        width={1200}
        identity="name"
        value="score"
        enableParentLabel={false}
        // enableLabel={false}
        leavesOnly={true}
        colors={{scheme: "spectral"}}
        nodeOpacity={0.6}
        borderWidth={0}
        label={"id"}
        labelTextColor={"black"}
      />
    </div>
  )
}

export default TreeMapChart
