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
        public DateTime? StartDate { get; set; } = DateTime.Now.ToUniversalTime() - TimeSpan.FromHours(1);
        public DateTime? StartTime { get; set; } = DateTime.Now.ToUniversalTime() - TimeSpan.FromHours(1);
        public DateTime? EndDate { get; set; } = DateTime.Now.ToUniversalTime();
        public DateTime? EndTime { get; set; } = DateTime.Now.ToUniversalTime();
        public bool SyncEndTime { get; set; } = true;

        public void RefreshEndTime()
        {
            var now = DateTime.Now.ToUniversalTime();
            EndDate = now;
            EndTime = now;
            StartDate = now - TimeSpan.FromHours(1);
            StartTime = now - TimeSpan.FromHours(1);
        }
    }
}
