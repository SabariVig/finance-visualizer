import { linearGradientDef } from "@nivo/core"
import { Line } from "@nivo/line"

const LineChart = ({ data, range="every 10 days" }) => {
  return (<div>
    <Line
      pointLabelYOffset={0}
      height={550}
      width={1200}
      data={data}
      margin={{ top: 40, right: 150, bottom: 80, left: 50 }}
      xScale={{
        type: 'time',
        format: '%Y-%m-%d',
        useUTC: false,
        precision: 'month',
      }}
      xFormat="time:%Y-%m-%d"
      yScale={{
        type: 'linear',
        stacked: false
      }}
      axisLeft={{
        legend: '',
        legendOffset: 12,
      }}
      axisBottom={{
        format: '%b %Y',
        tickValues: range,
        legend: '',
        legendOffset: -12,
      }}
      enableSlices={"x"}
      pointSize={0}
      lineWidth={1}
      colors={{ scheme: "dark2" }}
      enableArea={true}
      enableGridX={false}
      enableGridY={false}
      curve={"natural"}
      defs={[
        linearGradientDef('gradientA', [
          { offset: 0, color: 'inherit' },
          { offset: 100, color: 'inherit', opacity: 0 },
        ]),
      ]}
      fill={[{ match: '*', id: 'gradientA' }]}
      yFormat={v=> `₹${v}`}
    />
  </div>)
}

export default LineChart
