using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Metrix.Models.Frontend
{
    public class MetricBrowserModel
    {
        public string SelectedDataPoint { get; set; }
        public string SelectedDataGroup { get; set; }
        public DateTime? StartDate { get; set; }
        public DateTime? StartTime { get; set; }
        public DateTime? EndDate { get; set; }
        public DateTime? EndTime { get; set; }
    }
}
