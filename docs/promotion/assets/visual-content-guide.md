# Visual Content Guide for arrayops

This guide provides specifications and guidelines for creating visual content (charts, diagrams, images) for arrayops promotion materials.

## Performance Chart Creation Guidelines

### Chart Types

#### Bar Charts
- **Use For**: Comparing performance across implementations
- **Format**: Grouped bars for multiple implementations
- **Scale**: Linear or log scale depending on value range
- **Colors**: Use consistent color scheme

#### Line Charts
- **Use For**: Showing performance scaling with array size
- **Format**: Multiple lines for different implementations
- **Scale**: Log scale often appropriate
- **Labels**: Clear axis labels and legends

#### Speedup Charts
- **Use For**: Showing speedup factors
- **Format**: Bar chart with speedup multipliers
- **Scale**: Linear scale (1x baseline)
- **Highlight**: Clearly mark baseline (1x)

### Chart Specifications

#### Dimensions
- **Width**: 800-1200px (web), 1200-1600px (print)
- **Height**: 600-800px (web), 800-1000px (print)
- **Aspect Ratio**: 4:3 or 16:9
- **DPI**: 300 DPI minimum for print, 150 DPI for web

#### Typography
- **Font**: Sans-serif (Arial, Helvetica, or similar)
- **Title**: 18-24pt, bold
- **Axis Labels**: 12-14pt
- **Legend**: 10-12pt
- **Readable**: Ensure text is readable at intended size

#### Colors
- **Scheme**: Professional, accessible
- **Contrast**: High contrast for readability
- **Color-Blind Friendly**: Use patterns/textures if needed
- **Consistency**: Use same colors across all charts

### Chart Tools

- **Python**: matplotlib, plotly, seaborn
- **Online**: Google Sheets, Excel (for quick charts)
- **Design**: Canva, Figma (for styled charts)
- **Specialized**: D3.js (for interactive charts)

### Example Chart Code (matplotlib)

```python
import matplotlib.pyplot as plt
import numpy as np

# Data
operations = ['sum()', 'scale()', 'mean()']
python_times = [50.2, 80.5, 55.0]
arrayops_times = [0.5, 1.5, 0.6]
numpy_times = [1.1, 2.3, 1.2]

# Create chart
x = np.arange(len(operations))
width = 0.25

fig, ax = plt.subplots(figsize=(10, 6))
bars1 = ax.bar(x - width, python_times, width, label='Pure Python', color='#1f77b4')
bars2 = ax.bar(x, arrayops_times, width, label='arrayops', color='#2ca02c')
bars3 = ax.bar(x + width, numpy_times, width, label='NumPy', color='#ff7f0e')

ax.set_ylabel('Execution Time (ms)', fontsize=12)
ax.set_title('Performance Comparison: 1M int32 Array Operations', fontsize=14, fontweight='bold')
ax.set_xticks(x)
ax.set_xticklabels(operations, fontsize=11)
ax.legend(fontsize=11)
ax.set_yscale('log')
ax.grid(axis='y', alpha=0.3)

plt.tight_layout()
plt.savefig('performance_comparison.png', dpi=300, bbox_inches='tight')
```

## Architecture Diagram Specifications

### Diagram Types

#### System Architecture
- **Components**: Show major components
- **Flow**: Data flow between components
- **Labels**: Clear component labels
- **Relationships**: Show relationships clearly

#### Data Flow Diagram
- **Stages**: Processing stages
- **Data Types**: Show data at each stage
- **Operations**: Operations performed
- **Flow Direction**: Clear arrows showing flow

#### Component Diagram
- **Modules**: arrayops modules/components
- **Dependencies**: Dependencies between modules
- **Interfaces**: Interfaces and APIs
- **Relationships**: How components relate

### Diagram Tools

- **Drawing**: Excalidraw, draw.io, Lucidchart
- **Code-Based**: Mermaid, Graphviz, PlantUML
- **Design**: Figma, Adobe Illustrator
- **Online**: Diagrams.net, Creately

### Diagram Style Guidelines

- **Clean**: Simple, uncluttered design
- **Consistent**: Consistent style elements
- **Labeled**: Clear labels for all elements
- **Color-Coded**: Use colors meaningfully
- **Professional**: Professional appearance

### Example Architecture Diagram Elements

```
┌─────────────┐
│   Python    │
│  array.array│
└──────┬──────┘
       │
       │ Buffer Protocol
       │
┌──────▼──────┐
│   PyO3      │
│  (Rust)     │
└──────┬──────┘
       │
       │ Zero-Copy
       │
┌──────▼──────┐
│   Rust      │
│ Operations  │
└─────────────┘
```

## Logo/Branding Guidelines

### Logo Usage

- **Consistency**: Use consistently across materials
- **Size**: Minimum size for readability
- **Placement**: Consistent placement
- **Background**: Versions for light/dark backgrounds
- **Clear Space**: Maintain clear space around logo

### Color Scheme

- **Primary Colors**: Define primary brand colors
- **Secondary Colors**: Supporting colors
- **Accent Colors**: Highlight colors
- **Backgrounds**: Light and dark variants
- **Accessibility**: Ensure color contrast

### Typography

- **Headings**: Consistent heading font
- **Body Text**: Readable body font
- **Code**: Monospace font for code
- **Hierarchy**: Clear typographic hierarchy

## Social Media Image Sizes

### Platform Specifications

#### Twitter/X
- **Profile Picture**: 400x400px
- **Header Image**: 1500x500px
- **Post Image**: 1200x675px (16:9)
- **Card Image**: 1200x628px

#### LinkedIn
- **Profile Picture**: 400x400px
- **Cover Image**: 1584x396px
- **Post Image**: 1200x627px
- **Article Image**: 1200x627px

#### Reddit
- **Profile Picture**: 256x256px
- **Header Image**: Varies by subreddit
- **Post Image**: 1200x675px (16:9)

#### Facebook
- **Profile Picture**: 180x180px
- **Cover Photo**: 820x312px
- **Post Image**: 1200x630px

#### Instagram
- **Profile Picture**: 110x110px
- **Post Image**: 1080x1080px (square) or 1080x1350px (portrait)
- **Story Image**: 1080x1920px

#### YouTube
- **Channel Icon**: 800x800px
- **Channel Art**: 2560x1440px
- **Thumbnail**: 1280x720px

### Design Guidelines

- **Readable Text**: Text readable at small sizes
- **High Contrast**: High contrast for visibility
- **Brand Colors**: Use brand colors consistently
- **Simple Design**: Simple, clear design
- **Mobile-First**: Consider mobile viewing

## Chart Style Guidelines

### Style Elements

#### Grid Lines
- **Subtle**: Light grid lines
- **Purpose**: Aid reading, not distract
- **Style**: Dotted or light solid lines

#### Axis Labels
- **Clear**: Descriptive axis labels
- **Units**: Include units (ms, seconds, etc.)
- **Formatting**: Consistent number formatting

#### Legend
- **Clear**: Clear legend labels
- **Position**: Logical position (top, bottom, side)
- **Style**: Consistent with chart style

#### Data Points
- **Visible**: Clearly visible data points
- **Consistent**: Consistent marker styles
- **Colors**: Meaningful color choices

### Color Palettes

#### Recommended Palettes

**Professional Palette**:
- Primary: #1f77b4 (Blue)
- Success: #2ca02c (Green)
- Warning: #ff7f0e (Orange)
- Error: #d62728 (Red)
- Neutral: #7f7f7f (Gray)

**Accessible Palette**:
- Ensure WCAG AA contrast ratios
- Test with color-blind simulators
- Use patterns/textures if needed

## Visual Asset Checklist

### Before Publishing

- [ ] Correct dimensions for platform
- [ ] High resolution (300 DPI for print)
- [ ] Readable text at intended size
- [ ] Consistent branding
- [ ] Color accessibility checked
- [ ] No distortion or stretching
- [ ] Proper file format (PNG, SVG, JPG)
- [ ] Optimized file size (web)
- [ ] Alt text prepared (web accessibility)

## File Organization

### Directory Structure

```
assets/
├── charts/
│   ├── performance/
│   ├── comparisons/
│   └── benchmarks/
├── diagrams/
│   ├── architecture/
│   ├── data-flow/
│   └── component/
├── logos/
│   ├── full-color/
│   ├── black/
│   └── white/
└── social-media/
    ├── twitter/
    ├── linkedin/
    └── facebook/
```

### Naming Conventions

- **Descriptive**: Clear, descriptive names
- **Consistent**: Consistent naming pattern
- **Versioned**: Version numbers if needed
- **Formatted**: Include format in name (optional)

**Examples**:
- `performance_comparison_1M_int32.png`
- `architecture_diagram_v2.svg`
- `logo_full_color.png`

## Tools and Resources

### Chart Creation
- matplotlib (Python)
- plotly (Python, interactive)
- D3.js (JavaScript, interactive)
- Google Sheets (Quick charts)
- Excel (Business charts)

### Diagram Creation
- Excalidraw (Drawing)
- draw.io / diagrams.net (Diagramming)
- Mermaid (Code-based diagrams)
- Figma (Design)
- Lucidchart (Professional)

### Image Editing
- GIMP (Free, open source)
- Figma (Web-based)
- Canva (Web-based, templates)
- Adobe Photoshop (Professional)
- Adobe Illustrator (Vector graphics)

### Color Tools
- Coolors.co (Color palettes)
- Contrast Checker (Accessibility)
- Color Oracle (Color-blind simulation)

## Best Practices Summary

1. **Consistency**: Use consistent styles across all visuals
2. **Clarity**: Keep designs clear and readable
3. **Accessibility**: Ensure color accessibility
4. **Quality**: Use high resolution for intended use
5. **Branding**: Maintain consistent branding
6. **Optimization**: Optimize file sizes for web
7. **Documentation**: Document color codes and styles
8. **Versioning**: Keep versions of assets if iterating
9. **Organization**: Organize files logically
10. **Testing**: Test visuals at intended sizes

## Additional Resources

- [WCAG Guidelines](https://www.w3.org/WAI/WCAG21/quickref/) - Accessibility guidelines
- [matplotlib Gallery](https://matplotlib.org/stable/gallery/) - Chart examples
- [Color Contrast Checker](https://webaim.org/resources/contrastchecker/) - Accessibility tool
- [Social Media Image Sizes](https://sproutsocial.com/insights/social-media-image-sizes-guide/) - Platform specifications

